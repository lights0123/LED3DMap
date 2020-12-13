<template>
	<button
		class="flex bg-indigo-100 font-semibold duration-150 bg-opacity-75 hover:bg-opacity-100 transition px-4 py-2 rounded-full justify-center"
		:class="
			disabled
				? 'text-indigo-400 cursor-not-allowed'
				: 'text-indigo-700 hover:bg-indigo-600 hover:text-white'
		"
		:disabled="disabled"
		v-on="$listeners"
	>
		<slot />
	</button>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'nuxt-property-decorator';

@Component
export default class Button extends Vue {
	@Prop({ type: Boolean, default: false }) disabled!: boolean;
}
</script>
