<template>
	<div>
		<h2 class="text-3xl font-semibold">Output</h2>
		<h3 class="text-2xl font-semibold">Outlier Correction</h3>
		<span class="leading-tight">Minimum brightness difference</span>
		<small class="block leading-tight mb-1 max-w-sm">
			The minimum difference between the base frame and lit frame to be considered "correct".
			If a frame is below this threshold, it will be assumed to have been hidden from the
			camera. These frames will be corrected with data from surrounding frames.
		</small>
		<number-input v-model="minBrightnessInput" :min="0" :max="254" :step="1" />
		<p>{{ affectedFrames }} affected frames</p>
		<Button
			class="items-center mt-2"
			:disabled="minBrightness === minBrightnessInput"
			@click="correct"
		>
			Correct
		</Button>
		<h3 class="mt-2 text-2xl font-semibold">Output Scaling</h3>
		<div class="sm:grid grid-cols-3 max-w-max gap-4">
			<div>
				<span class="leading-tight">X scale</span>
				<number-input
					v-model="xScale"
					:min="0.001"
					:max="100"
					:step="0.001"
					:button-step="0.01"
				/>
			</div>
			<div>
				<span class="leading-tight">Y scale</span>
				<number-input
					v-model="yScale"
					:min="0.001"
					:max="100"
					:step="0.001"
					:button-step="0.01"
				/>
			</div>
			<div>
				<span class="leading-tight">Z scale</span>
				<number-input
					v-model="zScale"
					:min="0.001"
					:max="100"
					:step="0.001"
					:button-step="0.01"
				/>
			</div>
			<div>
				<span class="leading-tight">X offset</span>
				<number-input v-model="xOffset" :step="0.001" :button-step="0.01" />
			</div>
			<div>
				<span class="leading-tight">Y offset</span>
				<number-input v-model="yOffset" :step="0.001" :button-step="0.01" />
			</div>
			<div>
				<span class="leading-tight">Z offset</span>
				<number-input v-model="zOffset" :step="0.001" :button-step="0.01" />
			</div>
		</div>
		<h3 class="my-2 text-2xl font-semibold">Preview</h3>
		<div class="h-screen">
			<three-preview :data="finalData" />
		</div>
		<h3 class="my-2 text-2xl font-semibold">Download</h3>
		<div class="lg:flex lg:space-x-3">
			<Button class="items-center mt-4" @click="downloadRaw">
				<Download class="inline mr-2" />
				Download raw data as JSON
			</Button>
			<Button class="items-center mt-4" @click="downloadJson">
				<Download class="inline mr-2" />
				Download 3D points as JSON
			</Button>
			<Button class="items-center mt-4" @click="downloadCsv">
				<Download class="inline mr-2" />
				Download 3D points as CSV
			</Button>
		</div>
	</div>
</template>

<script lang="ts">
import { Vue, Component, Watch, Prop } from 'nuxt-property-decorator';
import { saveAs } from 'file-saver';
import { ProcessedData } from '@/pages/index.vue';
import type { FrameInfo } from '@/rs/led-map-wasm/pkg';
import type LedMapWasm from '@/rs/led-map-wasm/pkg';
import Download from '!!vue-svg-loader!feather-icons/dist/icons/download.svg';
interface ExtraFrameInfo extends FrameInfo {
	initialX: number;
	initialY: number;
}

export interface FinalData {
	x: number;
	y: number;
	z: number;
}

function correctFrames(framesIn: FrameInfo[], minBrightness: number): ExtraFrameInfo[] {
	const frames = framesIn.map((frame) => ({ ...frame })) as (FrameInfo &
		Partial<ExtraFrameInfo>)[];
	let min = -1;
	let max = -1;
	for (let i = 0; i < frames.length; i++) {
		const frame = frames[i];
		frame.initialX = frame.x;
		frame.initialY = frame.y;
		if (frame.maxBrightness <= minBrightness) {
			if (min >= 0) max = i;
			else min = max = i;
		} else if (min >= 0) {
			const initialX = (min ? frames[min - 1] : frames[i]).x;
			const initialY = (min ? frames[min - 1] : frames[i]).y;
			const finalX = frames[i].x;
			const finalY = frames[i].y;
			const diff = max - min + 2;
			for (let j = min; j <= max; j++) {
				frames[j].initialX = frames[j].x;
				frames[j].initialY = frames[j].y;
				frames[j].x = initialX + (finalX - initialX) * ((j - min + 1) / diff);
				frames[j].y = initialY + (finalY - initialY) * ((j - min + 1) / diff);
			}
			min = max = -1;
		}
	}
	if (min >= 0) {
		const x = (min ? frames[min - 1] : frames[0]).x;
		const y = (min ? frames[min - 1] : frames[0]).y;
		for (let j = min; j <= max; j++) {
			frames[j].initialX = frames[j].x;
			frames[j].initialY = frames[j].y;
			frames[j].x = x;
			frames[j].y = y;
		}
	}
	return frames as ExtraFrameInfo[];
}

@Component({ components: { Download } })
export default class VideoOutput extends Vue {
	@Prop({ type: Object, required: true }) frames!: ProcessedData;
	minBrightnessInput = 19;
	minBrightness = 19;
	wasm: Promise<typeof LedMapWasm> | null = null;
	xScale = 1;
	yScale = 1;
	zScale = 1;
	xOffset = 0;
	yOffset = 0;
	zOffset = 0;

	mounted() {
		this.wasm = import('@/rs/led-map-wasm/pkg');
	}

	get affectedFrames() {
		let count = 0;
		for (let i = 0; i < this.frames.first.length; i++) {
			if (
				Math.min(this.frames.first[i].maxBrightness, this.frames.second[i].maxBrightness) <
				this.minBrightnessInput
			) {
				count++;
			}
		}
		return count;
	}

	correct() {
		this.minBrightness = this.minBrightnessInput;
	}

	get correctedFrames() {
		return {
			first: correctFrames(this.frames.first, this.minBrightness),
			second: correctFrames(this.frames.second, this.minBrightness),
		};
	}

	@Watch('correctedFrames', { immediate: true })
	async runLinReg(frames: VideoOutput['correctedFrames']) {
		const { first, second } = frames;
		if (!this.wasm) return;
		const m = await this.wasm;
		const slope = m.lin_reg(
			Float64Array.from(first, (frame) => frame.y),
			Float64Array.from(second, (frame) => frame.y)
		);
		this.yScale = slope;
	}

	get finalData() {
		const { first, second } = this.correctedFrames;
		const finalData: FinalData[] = Array(first.length);
		for (let i = 0; i < first.length; i++) {
			finalData[i] = {
				x: first[i].x * this.xScale + this.xOffset,
				y: second[i].x * this.yScale + this.yOffset,
				z: first[i].y * this.zScale + this.zOffset,
			};
		}
		return finalData;
	}

	downloadRaw() {
		saveAs(
			new Blob([JSON.stringify(this.correctedFrames)], {
				type: 'application/json;charset=utf-8',
			}),
			'raw.json'
		);
	}

	downloadJson() {
		saveAs(
			new Blob([JSON.stringify(this.finalData)], { type: 'application/json;charset=utf-8' }),
			'lights3d.json'
		);
	}

	downloadCsv() {
		saveAs(
			new Blob(
				['x,y,z\n' + this.finalData.map(({ x, y, z }) => `${x},${y},${z}`).join('\n')],
				{ type: 'text/csv;charset=utf-8' }
			),
			'lights3d.csv'
		);
	}
}
</script>

<style scoped></style>
