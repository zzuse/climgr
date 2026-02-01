import CommandList from "@/components/CommandList";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center p-8 bg-zinc-50 dark:bg-black">
        <h1 className="text-4xl font-bold mb-8 dark:text-white">CLI Manager</h1>
        <CommandList />
    </main>
  );
}
