import React from "react";
import { atom, useAtom } from "jotai";
// import { atomFamily } from "jotai/utils";


const Somethings = () => {
  console.log("render somethings");
   const [names ,setStuff] = useAtom(timeAtom);
  return (
    <div>
      {names.map((name, i) => {
        return <div key={i}>{name}</div>;
      })}
      <button onClick={()=>  setStuff((prev) => [...prev, new Date().toISOString()])}>Add Time</button>
    </div>
  );
};

export default Somethings;

export const timeAtom = atom<string[]>(["Oyelowo"]);

