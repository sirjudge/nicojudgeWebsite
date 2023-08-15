import React from "react";
import {ComponentPreview, Previews} from "@react-buddy/ide-toolbox";
import {PaletteTree} from "./palette";
import AboutMe from "../components/AboutMe";

const ComponentPreviews = () => {
    return (
        <Previews palette={<PaletteTree/>}>
            <ComponentPreview path="/AboutMe">
                <AboutMe/>
            </ComponentPreview>
        </Previews>
    );
};

export default ComponentPreviews;