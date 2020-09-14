import React from "react";
import styles from "./Modal.module.css";
import { Button } from "./Button";

export const Modal: React.FC = ({children}) => (
    <div className={ styles.modal }>
        <div className={ styles.modalContent }>
            { children }
        </div>
    </div>
)
Modal.displayName = "Modal";

interface IProps {
    msg: string | null;
    dismiss: () => void;
}

export const ErrorModal: React.FC<IProps> = ({msg, dismiss}) => (
    msg ? <Modal>
            <p>{ msg }</p>
            <Button title="Dismiss"
                onClick={ dismiss }
            >
                Dismiss
            </Button>
        </Modal>
        : null
);
ErrorModal.displayName = "ErrorModal";
