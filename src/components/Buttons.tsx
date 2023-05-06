import { Field, Focusable, GamepadButton } from "decky-frontend-lib";
import { CSSProperties } from "react"
import { FaArrowLeft, FaArrowRight } from "react-icons/fa";

const styles: { [key: string]: CSSProperties } = {
    wrapper: {
        position: 'absolute',
        width: '100%',
        zIndex: '100',
        top: '0',
        textAlign: 'center'
    },
    btn: {
        width: '30px',
        height: '30px',
        cursor: 'pointer',
        userSelect: 'none',
        position: 'absolute',
        bottom: '0',
        font: '16px/30px sans-serif',
        color: 'rgba(255,255,255,0.8)'
    },
    left: {
        left: '0'
    },
    right: {
        right: '0'
    }
}

interface WidgetProps {
    index: number;
    total: number;
    prevHandler: () => void;
    nextHandler: () => void;
    axis?: 'x' | 'y';
    auto?: boolean;
    loop?: boolean;
    interval?: number;
}

export default function Buttons({ index, total, loop, prevHandler, nextHandler }: WidgetProps) {
    const prevBtnStyle = Object.assign({}, styles.btn, styles.left)
    const nextBtnStyle = Object.assign({}, styles.btn, styles.right)
    return (
        <div style={styles.wrapper}>
            {(loop || index !== 0) && (
                <Field
                    onButtonDown={(evt) => evt.detail.button == GamepadButton.OK && prevHandler()}
                    highlightOnFocus={true}
                    focusable={false}
                    >
                    <Focusable
                        style={prevBtnStyle}
                        >
                        <FaArrowLeft />
                    </Focusable>
                </Field>
            )}
            {(loop || index !== total - 1) && (
                <Field
                    onButtonDown={(evt) => evt.detail.button == GamepadButton.OK && nextHandler()}
                    highlightOnFocus={true}
                    focusable={false}
                    >
                    <Focusable
                        style={nextBtnStyle}
                        >
                        <FaArrowRight />
                    </Focusable>
                </Field>
            )}
        </div>
    )
}