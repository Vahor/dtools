import { commands } from "@/commands";
import { Settings } from "lucide-react";
import { useEffect, useState } from "react";
import { cn } from "@/lib/utils";
import { ButtonTooltip } from "../ui/button-tooltip";
import { Link } from "@tanstack/react-router";

export const SidebarFooter = () => {


  return (
    <div className="flex flex-col gap-2 pb-2 w-full items-center">
      <ButtonTooltip tooltip="Settings" size="sm" asChild side="right">
        <Link to="/dashboard/settings">
          <Settings className="size-5" />
        </Link>
      </ButtonTooltip>
      <StatusIndicator />
    </div >
  );
}

const StatusIndicator = () => {

  const [lastPacketTimeStamp, setLastPacketTimeStamp] = useState<bigint | null>(null);
  const [version, setVersion] = useState<string | null>(null);


  useEffect(() => {
    const interval = setInterval(() => {
      commands.getLastPacketTimestamp().then((timestamp) => {
        setLastPacketTimeStamp(BigInt(timestamp));
      });
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    commands.getGlobalConfig().then((config) => {
      setVersion(config.game_version.version);
      console.log('config', config);
    });
  }, []);

  const now = BigInt(Date.now());
  const isActive = lastPacketTimeStamp !== null && lastPacketTimeStamp + BigInt(10000) > now;


  return (
    <ButtonTooltip tooltip={`v${version}` ?? "Loading..."} size="sm" side="right" className="cursor-default">
      <img src="/dofus-icon.png" alt="logo" className={cn("size-5", !isActive && 'grayscale')} />
    </ButtonTooltip>
  );
}

