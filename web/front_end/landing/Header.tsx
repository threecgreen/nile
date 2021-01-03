import React from "react";
import { CoverArt } from "components/CoverArt";
import styles from "./Header.module.css";

export const Header: React.FC = () => (
    <div className={ styles.header }>
        <CoverArt />
        <div className={ styles.headerText }>
            <h1 className={ styles.landingTitle }>nile</h1>
            <h2 className={ styles.subtitle }>a path-creating game</h2>
        </div>
    </div>
);
Header.displayName = "Header";
