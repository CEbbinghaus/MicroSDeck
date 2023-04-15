import {
    afterPatch,
    ServerAPI,
    wrapReactType,
    findInReactTree,
    appDetailsClasses,
    playSectionClasses
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
                (_: Record<string, unknown>[], ret?: ReactElement) => {
                    if (!ret?.props?.children?.type?.type) {
                        return ret
                    }

                    wrapReactType(ret.props.children)
                    afterPatch(
                        ret.props.children.type,
                        'type',
                        (_2: Record<string, unknown>[], ret2?: ReactElement) => {
                            const container = findInReactTree(
                                ret2,
                                (x: ReactElement) =>
                                    Array.isArray(x?.props?.children) &&
                                    x?.props?.className?.includes(
                                        playSectionClasses.ActionSection
                                    )
                            )
                            console.log("Found Container", container);
                            if (typeof container !== 'object') {
                                return ret2
                            }

                            // container.props.children.splice(
                            //     1,
                            //     0,
                            //     <SettingsProvider>
                            //         <ProtonMedal serverAPI={serverAPI} />
                            //     </SettingsProvider>
                            // )
                            container.props.children.splice(
                                1,
                                0,
                                <h1>Hello, World!</h1>
                            )

                            return ret2
                        }
                    )
                    return ret
                }
            )
            return props
        }
    )
}

export default PatchPlayButton
