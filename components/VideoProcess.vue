<template>
	<div>
		<h2 class="mb-2 text-3xl font-semibold">Processing</h2>
		<div class="bg-gray-300 rounded-full">
			<div :style="{ width: `${progress * 100}%` }" class="bg-blue-400 py-1 rounded-full" />
		</div>
		<p>{{ status }}</p>
	</div>
</template>

<script lang="ts">
import { Vue, Component, Prop } from 'nuxt-property-decorator';
import ComputeWorker from 'worker-loader!@/components/computeWorker';
import WorkerPool from '@/components/workerPool';
import type { VideoData } from '@/pages/index.vue';
import type { FrameInfo } from '@/rs/led-map-wasm/pkg';

@Component
export default class VideoProcess extends Vue {
	@Prop({ type: Object, required: true }) first!: VideoData;
	@Prop({ type: Object, required: true }) second!: VideoData;

	status = '';
	progress = 0;

	mounted() {
		this.loaded();
	}

	async loaded() {
		try {
			this.status = 'Processing first video...';
			const first = await this.extractFrames(this.first);
			this.status = 'Processing second video...';
			const second = await this.extractFrames(this.second);
			this.$emit('done', {
				first,
				second,
			});
		} catch (e) {
			console.error(e);
			this.status = `Error: ${e}`;
		}
	}

	async extractFrames(data: VideoData) {
		const video = document.createElement('video');
		video.muted = true;
		video.src = data.video;
		await new Promise((resolve) => (video.onloadedmetadata = resolve));
		let seekResolve: () => void | undefined;
		video.addEventListener('seeked', () => seekResolve?.());
		const canvas = document.createElement('canvas');
		const contextNullable = canvas.getContext('2d');
		if (!contextNullable) throw new Error('Failed to get 2d context');
		const context = contextNullable!;
		const [w, h] = [video.videoWidth, video.videoHeight];
		canvas.width = w;
		canvas.height = h;

		async function getFrame(time: number) {
			video.currentTime = time;
			await new Promise((resolve) => (seekResolve = resolve));
			context.drawImage(video, 0, 0, w, h);
			return new Uint8Array(context.getImageData(0, 0, w, h).data);
		}

		const pool = new WorkerPool(ComputeWorker, {
			id: 'new',
			width: w,
			height: h,
			img: await getFrame(data.darkFrame),
		});
		const finished: Promise<FrameInfo>[] = [];

		for (let i = 0; i < data.ledCount; i++) {
			this.progress = i / data.ledCount;
			await pool.waitQueue;
			finished.push(
				pool.run({
					id: 'frame',
					width: w,
					height: h,
					img: await getFrame(data.firstFrame + i * data.pixelRate),
				}) as any
			);
		}

		const res = await Promise.all(finished);
		pool.terminate();
		return res;
	}
}
</script>

<style scoped></style>
