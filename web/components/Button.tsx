import React from "react";

interface IProps {
    onClick: () => void;
    enabled?: boolean;
}

// TODO: use icons
export const Button: React.FC<IProps> = ({enabled, ...props}) => {
    enabled = enabled ?? true;
    const onClick = (e: React.MouseEvent) => {
        e.preventDefault();
        props.onClick();
    };

    return (
        <button onClick={ onClick }
            disabled={ !enabled }
        >
            { props.children }
        </button>
    );
};
Button.displayName = "Button";
