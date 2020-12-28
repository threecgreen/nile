import React from "react";
import { CoverArt } from "./CoverArt";
import styles from "./Header.module.css";

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
