import React from "react";
import FemaleIconNew from "@/icons/female-icon-new";
import { colors } from "@/utils/utils";

export default function Home() {
  return (
    <>
      {colors.map((color, index) => (
        <FemaleIconNew key={index} color={color} />
      ))}
    </>
  );
}
