import React from "react";
import { colors } from "@/utils/utils";
import MaleIcon from "@/icons/male-icon";
import FemaleIcon from "@/icons/female-icon";

export default function Home() {
  return (
    <div className="flex flex-row">
      <div className="flex flex-col">
        {colors.map((color, index) => (
          <MaleIcon key={index} color={color} />
        ))}
      </div>
      <div className="flex flex-col">
        {colors.map((color, index) => (
          <FemaleIcon key={index} color={color} />
        ))}
      </div>
    </div>
  );
}
