<template>
	<div>
		<Nuxt />
	</div>
</template>

<style>
html {
	font-family: 'Source Sans Pro', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto,
		'Helvetica Neue', Arial, sans-serif;
	font-size: 18px;
	word-spacing: 1px;
	-ms-text-size-adjust: 100%;
	-webkit-text-size-adjust: 100%;
	-moz-osx-font-smoothing: grayscale;
	-webkit-font-smoothing: antialiased;
	box-sizing: border-box;
	@apply text-gray-900;
}

*,
*::before,
*::after {
	box-sizing: border-box;
	margin: 0;
}

p a {
	@apply border-b-2 text-blue-500 border-gray-300 hover:border-blue-400;
}

p ul {
	@apply list-outside list-disc ml-4;
}
</style>
