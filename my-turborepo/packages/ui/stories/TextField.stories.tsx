import { ComponentMeta, ComponentStory } from "@storybook/react";
import React from "react";

import { TextField } from "../components/TextField";

export default {
  title: "Example/TextField",
  component: TextField,
} as ComponentMeta<typeof TextField>;

const Template: ComponentStory<typeof TextField> = args => <TextField {...args} />;

export const TextFieldi = Template.bind({});
TextFieldi.args = {
  labkel: "Oyelowo",
  description: "The calm guy",
  errorMessage: "Don't hurt me",
};
