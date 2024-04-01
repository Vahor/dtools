
import { Outlet, createFileRoute } from "@tanstack/react-router"
import { Sidebar } from "@/components/sidebar"

export const Route = createFileRoute('/dashboard/_layout')({
  component: DashboardLayout,
})

function DashboardLayout() {
  return (
    <>
      <Sidebar />
      <main className="bg-soft w-full flex">
        <Outlet />
      </main>
    </>
  )
}
