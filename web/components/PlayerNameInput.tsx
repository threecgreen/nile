import React from "react";

interface IProps {
    i: number;
    name: string;
    onChange: (text: string) => void;
}

export const PlayerNameInput: React.FC<IProps> = ({i, name, onChange}) => {
    return (
        <>
            <input id={ i.toString() }
                value={ name }
                onChange={ (e) => onChange(e.target.value) }
                required={ true }
            />
            <label htmlFor={ i.toString() }>
                Player { i + 1 }
            </label>
        </>
    );
}
PlayerNameInput.displayName = "PlayerNameInput";
