
import { Link, Outlet, createFileRoute } from "@tanstack/react-router"
import { commands } from "../../commands"

export const Route = createFileRoute('/dashboard/_layout')({
  component: DashboardLayout,
})

function DashboardLayout() {
  return (
    <div className="p-2">
      <Link to="/dashboard/settings">Settings</Link>
      <Link to="/dashboard">Dashboard</Link>
      <button onClick={() => commands.createChatWindow()}>
        popup
      </button>
      <Outlet />
    </div>
  )
}
