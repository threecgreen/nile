import React from "react";
import styles from "./Header.module.css";
import { CoverArt } from "./CoverArt";

export const Header: React.FC = () => (
    <div className={ styles.header }>
        <CoverArt />
        <section className={ styles.headerText }>
            <h1 className={ styles.landingTitle }>nile</h1>
            <h2 className={ styles.subtitle }>a path-creating game</h2>
        </section>
    </div>
);
Header.displayName = "Header";
