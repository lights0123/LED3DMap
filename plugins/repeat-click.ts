import Vue, { DirectiveOptions } from 'vue';

const isServer = Vue.prototype.$isServer;

const on = (function (): (element: any, event: any, handler: any) => void {
	if (!isServer && document.addEventListener) {
		return function (element, event, handler) {
			if (element && event && handler) {
				element.addEventListener(event, handler, false);
			}
		};
	} else {
		return function (element, event, handler) {
			if (element && event && handler) {
				element.attachEvent('on' + event, handler);
			}
		};
	}
})();

const off = (function (): (element: any, event: any, handler: any) => void {
	if (!isServer && document.removeEventListener) {
		return function (element, event, handler) {
			if (element && event) {
				element.removeEventListener(event, handler, false);
			}
		};
	} else {
		return function (element, event, handler) {
			if (element && event) {
				element.detachEvent('on' + event, handler);
			}
		};
	}
})();

function once(el: any, event: any, fn: any) {
	const listener = function (this: HTMLElement) {
		if (fn) {
			fn.apply(this, arguments);
		}
		off(el, event, listener);
	};
	on(el, event, listener);
}

export default {
	bind(el, binding, vnode) {
		let interval: undefined | ReturnType<typeof setInterval>;
		let startTime = 0;
		const handler = () => vnode.context![binding.expression as keyof Vue].apply();
		const clear = () => {
			if (Date.now() - startTime < 100) {
				handler();
			}
			clearInterval(interval!);
			interval = undefined;
		};

		on(el, 'mousedown', (e: MouseEvent) => {
			if (e.button !== 0) return;
			startTime = Date.now();
			once(document, 'mouseup', clear);
			clearInterval(interval!);
			interval = setInterval(handler, 100);
		});
	},
} as DirectiveOptions;
