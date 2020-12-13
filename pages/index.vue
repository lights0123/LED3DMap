<template>
	<div>
		<div class="container mx-auto p-4 mb-20">
			<h1 class="text-4xl font-semibold">LED3DMap</h1>
			<p>
				<a :href="repository" target="_blank" rel="noopener"> Source code </a>
				&bull;
				<a href="https://lights0123.com/" target="_blank" rel="noopener">
					Made by Ben Schattinger
				</a>
			</p>
			<p>{{ description }}</p>
			<h2 class="text-3xl font-semibold">Instructions</h2>
			<p>
				First, run a program that will make each light sequentially light up for a certain
				period of time. If using an Arduino or compatible device, there is a sample
				<a href="/LightMapper.ino" download>FastLED sketch</a> available. Now, record the
				sequence from two different camera angles, each 90° apart. The vertical axis of each
				recording must match, so make sure that if you had to rotate your camera, the videos
				must be rotated accordingly. A tripod is highly recommended to ensure consistency. A
				modern browser is required.
			</p>
			<h2 class="text-3xl font-semibold">Features</h2>
			<ul>
				<li>Fully client-side: your video is never uploaded</li>
				<li>Multithreaded: videos are processed with as many cores as your CPU has</li>
				<li>
					Error correction: lights unable to be seen from the camera are automatically
					detected and corrected using neighboring data
				</li>
				<li>
					Different output formats available: 3D data can be downloaded as JSON or CSV,
					and the raw data processed from the videos can be downloaded as JSON
				</li>
				<li>3D Preview: A 3D preview is shown, updating live with tweaked parameters</li>
			</ul>
			<h2 class="text-3xl font-semibold">Example</h2>
			I captured videos from a tree outside my house and got this very accurate 3D view:
			<video muted autoplay loop width="760" height="1012" class="max-h-screen w-96">
				<source src="@/assets/demo.webm" />
			</video>
		</div>
		<div ref="header" class="min-h-screen container mx-auto p-4">
			<video-select v-if="stage === 'first'" key="first" first @done="videoSelected" />
			<video-select
				v-else-if="stage === 'second'"
				key="second"
				:first-led-count="firstStage.ledCount"
				:first-pixel-rate="firstStage.pixelRate"
				@done="videoSelected"
			/>
			<video-process
				v-else-if="stage === 'process'"
				:first="firstStage"
				:second="secondStage"
				@done="processDone"
			/>
			<video-output v-else-if="stage === 'done'" :frames="processedData" />
		</div>
	</div>
</template>

<script lang="ts">
import { Vue, Component, Ref, Watch } from 'nuxt-property-decorator';
import { description, repository } from '@/package.json';
import type { FrameInfo } from '@/rs/led-map-wasm/pkg';
export interface VideoData {
	pixelRate: number;
	darkFrame: number;
	firstFrame: number;
	video: string;
	ledCount: number;
}
export interface ProcessedData {
	first: FrameInfo[];
	second: FrameInfo[];
}
@Component
export default class Index extends Vue {
	description = description;
	repository = repository;
	stage: 'first' | 'second' | 'process' | 'done' = 'first';
	firstStage: VideoData | null = null;
	secondStage: VideoData | null = null;
	processedData: ProcessedData | null = null;
	@Ref() readonly header!: HTMLDivElement;

	@Watch('stage')
	scrollBack() {
		this.header.scrollIntoView?.();
	}

	videoSelected(data: VideoData) {
		if (this.stage === 'first') {
			this.firstStage = data;
			this.stage = 'second';
		} else if (this.stage === 'second') {
			this.secondStage = data;
			this.stage = 'process';
		}
	}

	processDone(data: ProcessedData) {
		this.processedData = data;
		this.stage = 'done';
	}
}
</script>

<style scoped>
ul {
	@apply list-outside list-disc ml-4;
}
</style>
