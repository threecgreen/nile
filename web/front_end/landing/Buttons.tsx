import { Button } from "components/Button";
import colors from "components/colors.module.css";
import { c } from "lib/utils";
import React from "react";
import styles from "./Buttons.module.css";

interface ILinkButtonProps {
    href: string;
}

export const LinkButton: React.FC<ILinkButtonProps> = ({href, ...props}) => (
    <a href={ href }
        className={ c([styles.landingButton, colors.nileBlue, "btn"]) }
    >
        { props.children }
    </a>
);
LinkButton.displayName = "LinkButton";

interface IClickButtonProps {
    onClick: () => void;
}

export const ClickButton: React.FC<IClickButtonProps> = ({onClick, ...props}) => (
    <Button onClick={ onClick }
        className={ c([styles.landingButton, colors.riverTurquoiseBg]) }
    >
        { props.children }
    </Button>
)
ClickButton.displayName = "ClickButton";
