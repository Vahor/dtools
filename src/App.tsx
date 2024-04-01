import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { commands } from './commands.ts';
import "./App.css";
import { invoke } from "@tauri-apps/api/core";

function App() {

  useEffect(() => {
    commands.appReady();
  }, []);

  return (
    <AppInner />
  );
}

function AppInner() {
  return null;
}

export default App;
