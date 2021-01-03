import React from "react";
import styles from "./Header.module.css";

export const Header: React.FC = () => (
    <div className={ styles.header }>
        <h1 className={ styles.inGameTitle }>nile</h1>
    </div>
);
Header.displayName = "Header";
