import commonjs from '@rollup/plugin-commonjs';
import json from '@rollup/plugin-json';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import replace from '@rollup/plugin-replace';
import typescript from '@rollup/plugin-typescript';
import { defineConfig } from 'rollup';
import importAssets from 'rollup-plugin-import-assets';
import codegen from 'rollup-plugin-codegen';
import mdx from '@mdx-js/rollup'
import deckyPlugin from "@decky/rollup";
import externalGlobals from 'rollup-plugin-external-globals';
import { merge } from 'merge-anything';

import plugin from "./plugin.json" assert {type: "json"};

export default defineConfig(merge(
	deckyPlugin(),
	{
		plugins: [
			nodeResolve({ browser: true }),
			codegen.default(),
			mdx({ providerImportSource: '@mdx-js/react' }),
			commonjs(),
			json(),
			typescript(),
			externalGlobals({
				react: 'SP_REACT',
				'react-dom': 'SP_REACTDOM',
				'@decky/ui': 'DFL',
				'@decky/manifest': JSON.stringify(plugin)
			}),
			replace({
				preventAssignment: false,
				'process.env.NODE_ENV': JSON.stringify('production'),
			}),
			importAssets({
				publicPath: `http://127.0.0.1:1337/plugins/${plugin.name}/`
			})
		],
	}
));