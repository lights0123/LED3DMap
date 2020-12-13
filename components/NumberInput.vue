<template>
	<div class="bg-gray-200 text-gray-600 flex flex-row h-10 w-32 rounded-lg">
		<button
			v-repeat-click="decrease"
			class="hover:text-gray-700 hover:bg-gray-400 h-full w-24 rounded-l cursor-pointer"
		>
			<span class="text-2xl font-thin">−</span>
		</button>
		<div class="h-full">
			<input
				type="number"
				class="text-center h-full w-full bg-gray-200 font-semibold text-md hover:text-black focus:text-black md:text-basecursor-default text-gray-700"
				:min="min"
				:max="max"
				:step="step || 'any'"
				:value="value"
				@change="change"
			/>
		</div>
		<div class="h-full flex items-center">
			<span class="text-gray-700 font-semibold">
				<slot name="after" />
			</span>
		</div>
		<button
			v-repeat-click="increase"
			class="hover:text-gray-700 hover:bg-gray-400 h-full w-24 rounded-r cursor-pointer"
		>
			<span class="text-2xl font-thin">+</span>
		</button>
	</div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'nuxt-property-decorator';
import RepeatClick from '@/plugins/repeat-click';

@Component({
	directives: { RepeatClick },
})
export default class NumberInput extends Vue {
	@Prop({ type: Number }) value?: number;
	@Prop({ type: Number }) min?: number;
	@Prop({ type: Number }) max?: number;
	@Prop({ type: Number }) step?: number;
	@Prop({ type: Number }) buttonStep?: number;

	change(e: Event) {
		const target = e.target as HTMLInputElement;
		const data = Number.parseFloat(target.value);
		if (Number.isNaN(data)) {
			target.value = this.value?.toString() || '';
		} else {
			const validated = this.validateNum(data);
			this.$emit('input', validated);
			target.value = validated.toString();
		}
	}

	validateNum(num: number) {
		if (typeof this.max === 'number' && num > this.max) num = this.max;
		if (typeof this.min === 'number' && num < this.min) num = this.min;
		if (typeof this.step === 'number') {
			const rem = num % this.step;
			if (rem) {
				if (rem > this.step / 2) {
					num += this.step - rem;
				} else {
					num -= rem;
				}
			}
		}
		return num;
	}

	decrease() {
		if (typeof this.value === 'number') {
			this.$emit('input', this.validateNum(this.value - (this.buttonStep || this.step || 1)));
		} else {
			this.$emit('input', this.min || 0);
		}
	}

	increase() {
		if (typeof this.value === 'number') {
			this.$emit('input', this.validateNum(this.value + (this.buttonStep || this.step || 1)));
		} else {
			this.$emit('input', this.min || 0);
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
