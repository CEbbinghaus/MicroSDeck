import {
    afterPatch,
    ServerAPI,
    wrapReactType,
    findInReactTree,
    appDetailsClasses,
    playSectionClasses,
    wrapReactClass
} from 'decky-frontend-lib'
import { ReactElement } from 'react'

function PatchPlayButton(serverAPI: ServerAPI) {
    
    const path = '/library/app/:appid';
    console.log(`Patching ${path}`)

    return serverAPI.routerHook.addPatch(
        path,
        (props?: { path?: string; children?: ReactElement }) => {
            if (!props?.children?.props?.renderFunc) {
                return props
            }

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

    afterPatch(
        root.props.children.type,
        'type',
        (_2: Record<string, unknown>[], element?: ReactElement) => {
            // window.rootEl = element;

            const container = findInReactTree(element, v => v.type?.prototype?.onGameInfoToggle); 
        
            if (typeof container !== 'object') {
                return element
            }
            
            PatchPanelElement(container);

            return element;
        }
    )

    return root;
}

function PatchPanelElement(panel: any): any {
    
    console.log("Found Container", panel);
    
    //     debugger;
    // window.panel = panel;
    
    wrapReactClass(panel)

    afterPatch(
        panel.type.prototype,
        'render',
        (_2: Record<string, unknown>[], element?: ReactElement) => {

            const container = findInReactTree(element, v => v?.props?.setSections); 
        
            if (typeof container !== 'object') {
                return element
            }

            PatchAppDetailsOverview(container);

            return element;
        }
    )
    return panel;
}

function PatchAppDetailsOverview(panel: any): any {
    
    console.log("Found AppDetails", panel);
    
    //     debugger;
    // window.appDetailsOverview = panel;
    
    // wrapReactType(panel)

    afterPatch(
        panel,
        'type',
        (_2: Record<string, unknown>[], tmpEl?: ReactElement) => {

            let cache: ReactElement;

            afterPatch(
                tmpEl,
                'type',
                (_2: Record<string, unknown>[], element?: ReactElement) => {
                    if(cache)
                        return cache;

                    const container = findInReactTree(element, v => v?.props?.setSections && v?.type?.render); 
        
                    if (typeof container !== 'object') {
                        return element
                    }
        
                    PatchPlaySection(container);
        
                    return (cache = element);
                }
            )
            return tmpEl;
        }
    )
    return panel;
}

function PatchPlaySection(panel: any): any {
    
    console.log("Found AppDetails Section", panel);
    
    //     debugger;
    // window.appDetailsSection = panel;
    
    wrapReactType(panel)

    afterPatch(
        panel.type,
        'render',
        (_2: Record<string, unknown>[], element?: ReactElement) => {
            const container = findInReactTree(element, v => v?.props?.setSections && v?.type?.render); 
        
            if (typeof container !== 'object') {
                return element
            }

            PatchAppRow(container);

            return element;
        }
    )
    return panel;
}

function PatchAppRow(panel: any): any {
    
    console.log("Found AppRow Section", panel);
    
    //     debugger;
    // window.appRowSection = panel;
    
    wrapReactType(panel)

    afterPatch(
        panel.type,
        'render',
        (_2: Record<string, unknown>[], tmpEl?: ReactElement) => {
            // afterPatch(
            //     tmpEl.type,
            //     'render',
            //     (_2: Record<string, unknown>[], element?: ReactElement) => {
            //         window.el = element;
            //         return element
            //     }
            // )

            debugger;

            return tmpEl;
        }
    )
    return panel;
}



export default PatchPlayButton
