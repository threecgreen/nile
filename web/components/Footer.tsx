import React from "react";
import styles from "./Footer.module.css";
import { TileSvg } from "./TileSvg";
import { Tile } from "nile";
import { Container } from "./Container";
import { GIT_SHA, VERSION } from "../generated/constants";

export const Footer: React.FC = () => (
    <div className={ styles.footerBackground }>
        <Container>
            <div className={ styles.footerFlex }>
                <div>
                    <p className={ styles.copyright }>© 2020&ndash;2021 Carter Green</p>
                </div>
                <div>
                    <div className={ styles.logo }>
                        <TileSvg tile={ Tile.Universal} />
                    </div>
                </div>
                <div>
                    <p className={ styles.version }>
                        { `Version ${VERSION}-${GIT_SHA}` }
                    </p>
                </div>
            </div>
        </Container>
    </div>
);
Footer.displayName = "Footer";
