import type WebpackWorker from 'worker-loader!*';

type Task = [[(arg: any) => void, (arg: any) => void], any];
export type WorkerRequest = { type: 'init'; data?: any } | { type: 'cmd'; data?: any };

export default class WorkerPool {
	public starter?: any;
	public waitQueue = Promise.resolve();
	private waitQueueResolve?: () => void;
	private creator: typeof WebpackWorker;
	private threads: number;
	private freeWorkers: Worker[] = [];
	private usedWorkers: Worker[] = [];
	private taskQueue: Task[] = [];
	private holdOffSpawning?: Promise<void> | false;

	constructor(creator: typeof WebpackWorker, starter?: any, threads?: number | null) {
		this.creator = creator;
		this.threads = Math.max(1, threads || navigator.hardwareConcurrency || 2);
		this.starter = starter;
	}

	run(task: any) {
		return new Promise((resolve, reject) => {
			this.taskQueue.push([[resolve, reject], task]);
			this.dispatch();
		});
	}

	private dispatch() {
		while (true) {
			let worker: Worker;
			let task: Task;
			let promise = Promise.resolve();
			if (this.taskQueue.length && this.freeWorkers.length) {
				task = this.taskQueue.pop()!;
				worker = this.freeWorkers.pop()!;
			} else if (
				this.taskQueue.length &&
				this.freeWorkers.length + this.usedWorkers.length < this.threads
			) {
				task = this.taskQueue.pop()!;
				worker = new this.creator();
				promise = new Promise(async (resolve) => {
					if (this.holdOffSpawning) {
						await this.holdOffSpawning;
					}
					worker.postMessage({
						type: 'init',
						data: this.starter,
					} as WorkerRequest);
					worker.onmessage = ({ data }) => {
						if (data) this.starter = data;
						resolve();
					};
				});
				if (this.holdOffSpawning === undefined) {
					this.holdOffSpawning = promise.then(() => {
						this.holdOffSpawning = false;
					});
				}
			} else break;
			this.usedWorkers.push(worker);
			promise.then(() => {
				worker.postMessage({
					type: 'cmd',
					data: task[1],
				} as WorkerRequest);
				worker.onmessage = ({ data }) => {
					worker.onmessage = null;
					task[0][0](data);
					this.freeWorkers.push(worker);
					const i = this.usedWorkers.indexOf(worker);
					if (i !== -1) this.usedWorkers.splice(i, 1);
					this.dispatch();
				};
			});
		}
		if (this.taskQueue.length > 1) {
			if (!this.waitQueueResolve) {
				this.waitQueue = new Promise((resolve) => (this.waitQueueResolve = resolve));
			}
		} else {
			this.waitQueueResolve?.();
			this.waitQueueResolve = undefined;
		}
	}

	terminate() {
		for (const worker of this.freeWorkers) {
			worker.terminate();
		}
		this.freeWorkers = [];
		for (const worker of this.usedWorkers) {
			worker.terminate();
		}
		this.usedWorkers = [];
	}
}
