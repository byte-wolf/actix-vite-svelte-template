import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, searchForWorkspaceRoot } from 'vite';




/** @type {import('vite').UserConfig} */
export default ({mode}) => {
	console.log(mode);
	let devEnvSettings = {};
	if (mode === 'development') {
		devEnvSettings = {
			server: {
				port: 3000,
				proxy: {
					'/api': 'http://localhost:8080'
				},
				fs: {
					// Allow serving files from one level up to the project root
					allow: [
						searchForWorkspaceRoot(process.cwd()),
					],
				},
			},
		}
	}

	console.log(searchForWorkspaceRoot(process.cwd()));

	return defineConfig({
		plugins: [sveltekit()],
		resolve: {
			preserveSymlinks: false
		},
		server: {
			fs: {
				// Allow serving files from one level up to the project root
				allow: [
					searchForWorkspaceRoot(process.cwd()),
				],
			},
		},
		...devEnvSettings
	})
};
