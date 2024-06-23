// utils/utils.js
import chroma from "chroma-js";

export const colors = [
  "#ff5a44",
  "#02de6d",
  "#08bcfe",
  "#9c08fe",
  "#feca08",
  "#08feea",
  "#fe08c1",
  "#d908fe",
];

export const generateGradient = (baseColor) => {
  try {
    const scale = chroma.scale([baseColor, "#ffffff"]).colors(5);
    return scale;
  } catch (error) {
    console.error(`Error generating shades for color ${baseColor}:`, error);
    return [baseColor];
  }
};
