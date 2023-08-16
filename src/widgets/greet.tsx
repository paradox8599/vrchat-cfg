"use client";

import { invoke } from "@tauri-apps/api/tauri";
import React from "react";

export default function Greet() {
  const [msg, setMsg] = React.useState("");
  async function onClick() {
    invoke<string>("greet").then(setMsg).catch(console.error);
  }


  return (
    <>
      <div className="text-center">
        <button className="py-4" onClick={onClick}>Test</button>
        <div>{msg}</div>
      </div>
    </>
  );
}
