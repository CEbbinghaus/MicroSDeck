// import { ServerAPI } from 'decky-frontend-lib'
import { ReactElement} from 'react'
import { FaSdCard } from 'react-icons/fa'

export default function LibraryModal(): ReactElement {
    var data = null;

    // if (!data) return (<></>);

    return (
        <div
            className="microsdeck-app-modal"
            style={{ position: 'absolute', top: '210px', left: '20px' }}
        >
            <FaSdCard size={20} />
            <span>
                {"Hello, World!"}
            </span>
        </div>
    )
}