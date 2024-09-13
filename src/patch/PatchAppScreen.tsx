import {
	afterPatch,
	createReactTreePatcher,
	findInReactTree,
	appDetailsClasses
} from '@decky/ui'
import { ReactElement } from 'react'
import { routerHook } from '@decky/api';
import LibraryModal from '../components/LibraryModal';
import { Logger } from '../Logging';
import { MicroSDeck, MicroSDeckContextProvider } from '../../lib/src';

function PatchLibraryApp(microSDeck: MicroSDeck) {
	
	const path = '/library/app/:appid';
	Logger.Log("Patching {path}", { path });

	return routerHook.addPatch(
		path,
		(tree: any) => {			
			Logger.Log("starting patch...", { tree });
			const routeProps = findInReactTree(tree, (x: any) => x?.renderFunc);
			
			if (routeProps) {

				Logger.Log("Found Props", { tree });
				
				const patchHandler = createReactTreePatcher([
					(tree: any) => findInReactTree(tree, (x: any) => x?.props?.children?.props?.overview)?.props?.children
				], (_: Array<Record<string, unknown>>, element?: ReactElement) => {
					const appId  = findInReactTree(element, (x: any) => x?.appid)?.appid;

					
					Logger.Log("Found AppId", { appId });

					if (!appId) {
						return element;
					}


					const container = findInReactTree(
						element,
						(x: ReactElement) =>
							Array.isArray(x?.props?.children) &&
							x?.props?.className?.includes(
								appDetailsClasses.InnerContainer
							)
					)

					if (typeof container !== 'object') {
						return element
					}

					Logger.Log("Found Appropriate location to patch.", { element, container, appId });

					container.props.children.splice(
						1,
						0,
						<MicroSDeckContextProvider microSDeck={microSDeck}>
							<LibraryModal appId={appId}/>
						</MicroSDeckContextProvider>,
					)

					return element
				});

				afterPatch(routeProps, "renderFunc", patchHandler);
			}

			return tree;
		}
	)
}

export default PatchLibraryApp
