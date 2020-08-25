import React from "react";

interface IProps {
    onClick: () => void;
    className?: string;
    enabled?: boolean;
    title?: string;
}

export const Button: React.FC<IProps> = ({className, enabled, title, ...props}) => {
    enabled = enabled ?? true;
    const onClick = (e: React.MouseEvent) => {
        e.preventDefault();
        props.onClick();
    };

    return (
        <button onClick={ onClick }
            className={ className }
            disabled={ !enabled }
            title={ title }
        >
            { props.children }
        </button>
    );
};
Button.displayName = "Button";
