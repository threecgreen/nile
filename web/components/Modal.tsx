import React from "react";
import styles from "./Modal.module.css";

export const Modal: React.FC = ({children}) => (
    <div className={ styles.modal }>
        <div className={ styles.modalContent }>
            { children }
        </div>
    </div>
)
Modal.displayName = "Modal";
