import React from "react"
import styles from "./Container.module.css";

export const Container: React.FC = (props) => (
    <div className={ styles.container }>
        { props.children }
    </div>
);
Container.displayName = "Container";