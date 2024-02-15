import commonjs from '@rollup/plugin-commonjs';
import json from '@rollup/plugin-json';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import replace from '@rollup/plugin-replace';
import typescript from '@rollup/plugin-typescript';
import { defineConfig } from 'rollup';
import importAssets from 'rollup-plugin-import-assets';
import codegen from 'rollup-plugin-codegen';
import mdx from '@mdx-js/rollup'

import plugin from "./plugin.json" assert {type: "json"};

export default defineConfig({
	input: './src/index.tsx',
	plugins: [
		nodeResolve({ browser: true }),
		codegen.default(),
		mdx({providerImportSource: '@mdx-js/react'}),
		commonjs(),
		typescript(),
		json(),
		replace({
			preventAssignment: false,
			'process.env.NODE_ENV': JSON.stringify('production'),
		}),
		importAssets({
			publicPath: `http://127.0.0.1:1337/plugins/${plugin.name}/`
		})
	],
	context: 'window',
	external: ['react', 'react-dom', 'decky-frontend-lib'],
	output: {
		file: 'dist/index.js',
		globals: {
			react: 'SP_REACT',
			'react-dom': 'SP_REACTDOM',
			'decky-frontend-lib': 'DFL',
		},
		format: 'iife',
		exports: 'default',
	},
});