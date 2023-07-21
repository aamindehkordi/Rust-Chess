import App from './App.svelte';
import wasm from '../../backend/Cargo.toml';

const init = async () => {
      const bindings = await wasm();
      const app = new App({
      target: document.body,
      props: {
                bindings,
              },
    });
};

init();

const app = new App({
	target: document.body,
	props: {
		name: 'world'
	}
});

export default app;