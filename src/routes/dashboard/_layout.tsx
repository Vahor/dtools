
import { Link, Outlet, createFileRoute } from "@tanstack/react-router"
import { commands } from "../../commands"
import { Sidebar } from "@/components/sidebar"

export const Route = createFileRoute('/dashboard/_layout')({
  component: DashboardLayout,
})

function DashboardLayout() {
  return (
    <>
      <Sidebar />
      <main className="pl-2">
        <Link to="/dashboard/settings">Settings</Link>
        <Link to="/dashboard">Dashboard</Link>
        <button onClick={() => commands.createChatWindow()}>
          popup
        </button>
        <Outlet />
      </main>
    </>
  )
}
