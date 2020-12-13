<template>
	<div ref="wrapper" class="canvas" />
</template>

<script lang="ts">
import { Component, Prop, Ref, Vue, Watch } from 'nuxt-property-decorator';
import ResizeObserver from 'resize-observer-polyfill';
import * as THREE from 'three';
import { OrbitControls } from 'babel-loader!three/examples/jsm/controls/OrbitControls';
import type { FinalData } from '@/components/VideoOutput.vue';

@Component
export default class ThreePreview extends Vue {
	@Prop({ type: Array, required: true }) data!: FinalData[];
	@Ref() readonly wrapper!: HTMLCanvasElement;
	resizeObserver?: ResizeObserver;
	destroyCb: (() => void)[] = [];
	scene: THREE.Scene | null = null;
	camera: THREE.PerspectiveCamera | null = null;
	controls: OrbitControls | null = null;
	renderer: THREE.Renderer | null = null;
	animationFrame = 0;
	active = true;

	activated() {
		if (!this.active) {
			this.active = true;
			this.animationFrame = requestAnimationFrame(this.draw);
		}
	}

	deactivated() {
		this.active = false;
	}

	setSize() {
		if (!this.wrapper || !this.camera) return;
		this.renderer?.setSize(this.wrapper.clientWidth, this.wrapper.clientHeight);
		this.camera.aspect = this.wrapper.clientWidth / this.wrapper.clientHeight;
		this.camera.updateProjectionMatrix();
	}

	geom: THREE.BufferGeometry | null = null;
	pos: THREE.Float32BufferAttribute | null = null;
	color: THREE.Float32BufferAttribute | null = null;

	@Watch('data', { immediate: true })
	dataUpdate(data: FinalData[]) {
		if (!this.scene) return;
		const geometry = this.geom || new THREE.BufferGeometry();

		const positionsBuffer =
			this.pos || new THREE.Float32BufferAttribute(new Float32Array(data.length * 3), 3);
		positionsBuffer.needsUpdate = true;
		const positions = positionsBuffer.array as Float32Array;

		for (let i = 0; i < data.length; i++) {
			const { x, y, z } = data[i];

			positions[i * 3] = x;
			positions[i * 3 + 1] = y;
			positions[i * 3 + 2] = z;
		}
		const { x, y, z } = data[0];
		const vertex = new THREE.Vector3(x, y, z);
		this.camera?.lookAt(vertex);
		if (this.controls) {
			this.controls.target = vertex;
		}
		this.controls?.update();

		if (!this.geom) {
			geometry.setAttribute('position', positionsBuffer);
			const colors = new Float32Array(data.length * 3);
			const color = new THREE.Color();
			for (let i = 0; i < data.length; i++) {
				color.setHSL(i / data.length, 1, 0.5);
				colors[i * 3] = color.r;
				colors[i * 3 + 1] = color.g;
				colors[i * 3 + 2] = color.b;
			}
			const colorBuffer = new THREE.Float32BufferAttribute(colors, 3);
			geometry.setAttribute('color', colorBuffer);
			const material = new THREE.PointsMaterial({ size: 15, vertexColors: true });

			const points = new THREE.Points(geometry, material);
			this.scene.add(points);
			this.geom = geometry;
			this.pos = positionsBuffer;
			this.color = colorBuffer;
		}
	}

	mounted() {
		this.scene = new THREE.Scene();
		this.camera = new THREE.PerspectiveCamera(75, 1, 0.1, 5000);
		this.camera.up.set(0, 0, -1);
		this.camera.position.set(0, -5, 3);
		this.setSize();
		this.renderer = new THREE.WebGLRenderer({ antialias: true });
		this.wrapper.appendChild(this.renderer.domElement);
		this.controls = new OrbitControls(this.camera, this.renderer.domElement);
		this.scene.add(new THREE.AmbientLight(0x555555, 1.2));
		const directionalLight = new THREE.DirectionalLight(0xffffff, 0.66);
		const directionalLight2 = new THREE.DirectionalLight(0xffffff, 0.66);
		directionalLight2.position.set(-1, -1, 0);
		this.scene.add(directionalLight);
		this.scene.add(directionalLight2);

		this.animationFrame = requestAnimationFrame(this.draw);
		window.addEventListener('resize', this.setSize);
		this.resizeObserver = new ResizeObserver(this.setSize);
		this.resizeObserver.observe(this.wrapper);
		this.dataUpdate(this.data);
	}

	draw() {
		if (this.active) this.animationFrame = requestAnimationFrame(this.draw);
		if (!this.renderer || !this.scene || !this.camera) return;
		this.renderer.render(this.scene, this.camera);
		this.controls?.update();
	}

	beforeDestroy() {
		cancelAnimationFrame(this.animationFrame);
		this.resizeObserver?.unobserve(this.wrapper);
		this.active = false;
		for (const cb of this.destroyCb) {
			cb();
		}
	}
}
</script>

<style scoped>
.canvas {
	width: 100%;
	height: 100%;
}
</style>
