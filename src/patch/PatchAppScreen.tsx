import {
	afterPatch,
	ServerAPI,
	wrapReactType,
	findInReactTree,
	appDetailsClasses
} from 'decky-frontend-lib'
import { ReactElement } from 'react'
import LibraryModal from '../components/LibraryModal';
import { Logger } from '../Logging';

function PatchAppScreen(serverAPI: ServerAPI) {

	const path = '/library/app/:appid';
	Logger.Log("Patching {path}", { path });

	return serverAPI.routerHook.addPatch(
		path,
		(props?: { path?: string; children?: ReactElement }) => {
			if (!props?.children?.props?.renderFunc) {
				return props
			}

			Logger.Log("patching...", { props });

			afterPatch(
				props.children.props,
				'renderFunc',
				(_: Record<string, unknown>[], element?: ReactElement) => {
					if (!element?.props?.children?.type?.type) {
						return element
					}

					return PatchRootElement(element);
				}
			)
			return props
		}
	)
}

function PatchRootElement(root: any): any {

	wrapReactType(root.props.children)

	const appDetails = root?.props?.children?.props?.overview || {};

	afterPatch(
		root.props.children.type,
		'type',
		(_2: Record<string, unknown>[], element?: ReactElement) => {
			// window.rootEl = element;

			// const container = findInReactTree(element, v => v.type?.prototype?.onGameInfoToggle); 

			// if (typeof container !== 'object') {
			//     return element
			// }

			// PatchPanelElement(container);

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

			Logger.Log("Found Appropriate location to patch.", { root, element, container, appDetails });

			container.props.children.splice(
				1,
				0,
				<LibraryModal appId={appDetails?.appid?.toString()} />
			)

			return element;
		}
	)

	return root;
}

export default PatchAppScreen;
