declare module 'worker-loader!*' {
	// You need to change `Worker`, if you specified a different value for the `workerType` option
	class WebpackWorker extends Worker {
		constructor();
	}

	// Uncomment this if you set the `esModule` option to `false`
	// export = WebpackWorker;
	export default WebpackWorker;
}

declare module '!!vue-svg-loader*' {
	import Vue from 'vue';
	export default Vue;
}

declare module 'babel-loader!three/examples/jsm/controls/OrbitControls' {
	import * as m from 'three/examples/jsm/controls/OrbitControls';
	export = m;
}