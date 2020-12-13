<template>
	<div>
		<h2 v-if="first" class="text-3xl font-semibold">First video</h2>
		<h2 v-else class="text-3xl font-semibold">Second video</h2>
		<h3 class="text-2xl font-semibold">First, let's set some parameters.</h3>
		<div class="block my-2">
			<div v-if="first">
				<span>Number of lights</span>
				<number-input v-model="ledCount" :min="1" :step="1" class="mb-4" />
			</div>
			<span class="leading-tight">Seconds per light</span>
			<small class="block leading-3 mb-1">
				The number of seconds that each light is lit for.
			</small>
			<number-input v-model="pixelRate" :min="0.001" :button-step="0.05" />
		</div>
		<div class="my-4">
			<h3 class="text-2xl font-semibold">Now, upload your video.</h3>
			<p v-if="first">This first video will define the X and Z axis.</p>
			<p v-else>This second video will define the Y axis.</p>
			The vertical axis will become the Z axis, so both videos "share" their vertical axis.
		</div>
		<input type="file" accept="video/*" @change="previewFiles" />
		<div v-if="video">
			<video ref="videoElement" :src="video" controls class="max-h-screen" />
			<h3 class="text-2xl font-semibold">
				<Check v-if="typeof darkFrame === 'number'" class="inline text-green-500" />
				Select a frame where no lights are lit up.
			</h3>
			<Button @click="setDarkFrame">
				{{ typeof darkFrame === 'number' ? 'Unselect frame' : 'Select this frame' }}
			</Button>
			<h3 class="mt-4 text-2xl font-semibold">
				<Check v-if="typeof firstFrame === 'number'" class="inline text-green-500" />
				Select a frame where the first LED is lit.
			</h3>
			<Button @click="setFirstFrame">
				{{ typeof firstFrame === 'number' ? 'Unselect frame' : 'Select this frame' }}
			</Button>
			<Button
				class="items-center mt-4"
				:disabled="typeof darkFrame !== 'number' || typeof firstFrame !== 'number'"
				@click="next"
			>
				Next
				<ArrowRight class="inline ml-2" />
			</Button>
		</div>
	</div>
</template>

<script lang="ts">
import { Vue, Component, Ref, Watch, Prop } from 'nuxt-property-decorator';
import Check from '!!vue-svg-loader!feather-icons/dist/icons/check.svg';
import ArrowRight from '!!vue-svg-loader!feather-icons/dist/icons/arrow-right.svg';

@Component({ components: { Check, ArrowRight } })
export default class VideoSelect extends Vue {
	@Prop({ type: Boolean, default: false }) first!: boolean;
	@Prop({ type: Number }) firstLedCount?: number;
	@Prop({ type: Number }) firstPixelRate?: number;
	video: string | null = null;
	ledCount = 60;
	pixelRate = 0.2;
	darkFrame: number | null = null;
	firstFrame: number | null = null;
	@Ref() readonly videoElement?: HTMLVideoElement;

	created() {
		if (this.firstLedCount) this.ledCount = this.firstLedCount;
		if (this.firstPixelRate) this.pixelRate = this.firstPixelRate;
	}

	previewFiles(event: Event) {
		const files = (event.target as HTMLInputElement).files;
		if (!files?.length) return;
		const file = files[0];
		this.video = URL.createObjectURL(file);
	}

	@Watch('video')
	freeVideo(_video: string | null, old: string | null) {
		if (old) URL.revokeObjectURL(old);
	}

	setDarkFrame() {
		if (typeof this.darkFrame === 'number') this.darkFrame = null;
		else {
			this.darkFrame = this.videoElement!.currentTime;
		}
	}

	setFirstFrame() {
		if (typeof this.firstFrame === 'number') this.firstFrame = null;
		else {
			this.firstFrame = this.videoElement!.currentTime;
		}
	}

	next() {
		if (
			typeof this.darkFrame === 'number' &&
			typeof this.firstFrame === 'number' &&
			this.video
		) {
			this.$emit('done', {
				ledCount: this.ledCount,
				pixelRate: this.pixelRate,
				darkFrame: this.darkFrame,
				firstFrame: this.firstFrame,
				video: this.video,
			});
		}
	}
}
</script>

<style scoped>
input[type='number']::-webkit-inner-spin-button,
input[type='number']::-webkit-outer-spin-button {
	-webkit-appearance: none;
	margin: 0;
}

input[type='number'] {
	-moz-appearance: textfield;
}
</style>
