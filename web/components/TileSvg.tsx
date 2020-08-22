import React, { CSSProperties } from "react";
import { Tile } from "nile";

interface ITileSvgProps {
    tile: Tile;
    strokeColor?: string;
}

export const TileSvg: React.FC<ITileSvgProps> = ({tile, strokeColor}) => {
    strokeColor = strokeColor ?? "#000000";
    switch (tile) {
        case Tile.Straight:
            return <Straight strokeColor={ strokeColor } />;
        case Tile.Diagonal:
            return <Diagonal strokeColor={ strokeColor } />;
        case Tile.Center90:
            return <Center90 strokeColor={ strokeColor } />;
        case Tile.Corner90:
            return <Corner90 strokeColor={ strokeColor } />;
        case Tile.Left45:
            return <Tile45
                strokeColor={ strokeColor }
                style={ {transform: reflectToCss(true)} }
            />;
        case Tile.Right45:
            return <Tile45 strokeColor={ strokeColor } />;
        case Tile.Left135:
            return <Tile135
                strokeColor={ strokeColor }
                style={ {transform: reflectToCss(true)} }
            />;
        case Tile.Right135:
            return <Tile135 strokeColor={ strokeColor } />;
        case Tile.Universal:
            return <Universal strokeColor={ strokeColor } />;
        default:
            throw new Error(`Unknown tile type: ${tile}`);
    }
}
TileSvg.displayName = "TileSvg";

const reflectToCss = (reflect: boolean): string => {
    return reflect ? "scaleX(-1)" : "";
}

interface ISvgProps {
    strokeColor: string;
    style?: CSSProperties;
}

const Straight: React.FC<ISvgProps> = ({strokeColor, style}) => (
    <SvgWrapper style={ style }>
        <line fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" x1="0" y1="20" x2="40" y2="20" />
    </SvgWrapper>
)
Straight.displayName = "Straight";
const Diagonal: React.FC<ISvgProps> = ({strokeColor, style}) => (
    <SvgWrapper style={ style }>
        <line fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" x1="40" y1="0" x2="0" y2="40" />
    </SvgWrapper>
)
Diagonal.displayName = "Diagonal";
const Center90: React.FC<ISvgProps> = ({strokeColor, style}) => (
    <SvgWrapper style={ style }>
        <path fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" d="M20,40c0-11.055-8.945-20-20-20"/>
    </SvgWrapper>
)
Center90.displayName = "Center90";
const Corner90: React.FC<ISvgProps> = ({strokeColor, style}) => (
    <SvgWrapper style={ style }>
        <path fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" d="M40,40C28.986,28.986,11.163,28.986,0.148,40"/>
    </SvgWrapper>
)
Corner90.displayName = "Corner90";
const Tile45: React.FC<ISvgProps> = ({strokeColor, style}) => (
    <SvgWrapper style={ style }>
        <path fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" d="M19.938,40.063c0-27.636,22.363-50,50-50" />
    </SvgWrapper>
)
Tile45.displayName = "Tile45";
const Tile135: React.FC<ISvgProps> = ({strokeColor, style}) => (
    <SvgWrapper style={ style }>
        <path fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" d="M0,40l15.725-15.725
            c0.444-0.527,1.11-0.862,1.854-0.862c1.337,0,2.422,1.084,2.422,2.422L20,40"
        />
    </SvgWrapper>
)
Tile135.displayName = "Tile135";
const Universal: React.FC<ISvgProps> = ({strokeColor, style}) => (
    <SvgWrapper style={ style }>
        <circle fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" cx="20" cy="20" r="5" />
        <line fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" x1="23.535" y1="23.535" x2="40" y2="40" />
        <line fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" x1="0" y1="0" x2="16.466" y2="16.466" />
        <line fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" x1="16.464" y1="23.535" x2="0" y2="40" />
        <line fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" x1="40" y1="0" x2="23.535" y2="16.464" />
        <line fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" x1="15" y1="20" x2="0" y2="20" />
        <line fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" x1="40" y1="20" x2="25" y2="20" />
        <line fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" x1="20" y1="25" x2="20" y2="40" />
        <line fill="none" stroke={ strokeColor } strokeWidth="3" strokeMiterlimit="10" x1="20" y1="0" x2="20" y2="15" />
    </SvgWrapper>
)
Universal.displayName = "Universal";

const SvgWrapper: React.FC<{style?: CSSProperties}> = (props) => (
    <svg
        // version="1.1"
        // id="Layer_1"
        // x="0px" y="0px"
        width="40"
        height="40"
        // viewBox="0 0 40 40"
        // enableBackground="new 0 0 40 40"
        style={ props.style ?? {} }
    >
        { props.children }
    </svg>
)
SvgWrapper.displayName = "SvgWrapper";
