import type { WorkerRequest } from './workerPool';
import type { BaseImage } from '@/rs/led-map-wasm/pkg';

const m = import('@/rs/led-map-wasm/pkg');
const ctx: Worker = self as any;
let baseImage: BaseImage | undefined;
ctx.onmessage = async ({ data }: { data: WorkerRequest }) => {
	const module = await m;
	if (data.type === 'init') {
		if (data.data.id === 'new') {
			baseImage = new module.BaseImage(data.data.width, data.data.height, data.data.img);
			const init = baseImage.inner();
			ctx.postMessage(
				{
					width: data.data.width,
					height: data.data.height,
					img: init,
				}
			);
		} else {
			baseImage = module.BaseImage.from_init(
				data.data.width,
				data.data.height,
				data.data.img
			);
			ctx.postMessage(undefined);
		}
	} else {
		if (data.data.id === 'frame') {
			ctx.postMessage(
				baseImage?.compute_frame(data.data.width, data.data.height, data.data.img)
			);
		}
	}
};
