import React from "react";
import styles from "./ShortcutsHelp.module.css";
import { Modal } from "./Modal";
import { Button } from "./Button";

/**
 * Keep updated with keydown event handlers in Game.tsx
 */
const BINDINGS: Array<[string, string]> = [
    ["q", "rotate counter-clockwise"],
    ["e", "rotate clockwise"],
    ["x", "remove tile"],
    ["u", "undo"],
    ["r", "redo"],
    ["E", "end turn"],
    ["C", "can't play"],
];

export const ShortcutsHelp: React.FC = () => (
    <table className={ styles.shortcutsHelp }>
        <tbody>
            <>
                { BINDINGS.map(([key, helpText]) => (
                    <tr key={ key }>
                        <td><span className={ styles.helpKey }>{ key }</span></td>
                        <td>{ helpText }</td>
                    </tr>
                )) }
            </>
            <tr>
                <td><span className={ styles.helpKey }>1</span>&ndash;<span className={ styles.helpKey }>5</span></td>
                <td>select the n<sup>th</sup> tile from the tile rack</td>
            </tr>
        </tbody>
    </table>
)
ShortcutsHelp.displayName = "ShortcutsHelp";

interface IModalProps {
    dismiss: () => void;
}

export const ShortcutsHelpModal: React.FC<IModalProps> = ({dismiss}) => {
    return (
        <Modal>
            <h2>Keyboard shortcuts</h2>
            <section>
                <ShortcutsHelp />
            </section>
            <Button title="Dismiss"
                onClick={ dismiss }
            >
                Dismiss
            </Button>
        </Modal>
    );
}
ShortcutsHelpModal.displayName = "ShortcutsHelpModal";
