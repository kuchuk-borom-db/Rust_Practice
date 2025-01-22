<script lang="ts">
    import { fade, fly } from 'svelte/transition';
    import { GitBranch, Globe, Share2 } from 'lucide-svelte';

    interface Feature {
        icon: any;
        title: string;
        description: string;
    }

    const features: Feature[] = [
        {
            icon: GitBranch,
            title: 'Visual Flow Analytics',
            description:
                'Transform complex logs into intuitive visual flows, making debugging and system analysis effortless. Trace issues and understand system behavior at a glance.',
        },
        {
            icon: Globe,
            title: 'Universal Integration',
            description:
                'Connect from any programming language using our comprehensive API libraries and drivers. No official SDK? No problem - integrate directly via REST API calls.',
        },
        {
            icon: Share2,
            title: 'Flexible Visualization',
            description:
                'Choose from multiple diagram types including Mermaid, React Flow, and more. Customize your log visualization to match your team\'s needs and preferences.',
        },
    ];
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-900 to-gray-800 text-white">
    <div class="container mx-auto px-6 py-16">
        <div
                transition:fly={{ y: 20, duration: 800 }}
                class="text-center"
        >
            <div class="mx-auto w-20 h-20 mb-8 bg-blue-500 rounded-lg flex items-center justify-center">
                <span class="text-2xl font-bold">VF</span>
            </div>

            <h1 class="text-5xl font-bold mb-4">VisFlow Logger</h1>
            <p class="text-xl text-gray-300 mb-8 max-w-2xl mx-auto">
                Revolutionize your debugging experience with visual log flows. Transform complex system logs into clear, actionable visualizations that make sense at first glance.
            </p>

            <button
                    class="bg-blue-500 text-white px-8 py-3 rounded-lg font-medium hover:bg-blue-600 transition-colors duration-200"
                    on:mouseenter={() => (event.target.style.transform = 'scale(1.05)')}
                    on:mouseleave={() => (event.target.style.transform = 'scale(1)')}
                    on:mousedown={() => (event.target.style.transform = 'scale(0.95)')}
                    on:mouseup={() => (event.target.style.transform = 'scale(1.05)')}
            >
                Get Started
            </button>
        </div>

        <div class="grid md:grid-cols-3 gap-8 mt-20">
            {#each features as feature, index}
                <div
                        transition:fly={{ y: 20, duration: 500, delay: index * 200 }}
                        class="bg-gray-800 p-6 rounded-lg hover:bg-gray-700 transition-colors duration-200"
                >
                    <div class="bg-blue-500/10 w-12 h-12 rounded-lg flex items-center justify-center mb-4">
                        <svelte:component this={feature.icon} class="w-6 h-6" />
                    </div>
                    <h3 class="text-xl font-semibold mb-2">{feature.title}</h3>
                    <p class="text-gray-400">{feature.description}</p>
                </div>
            {/each}
        </div>

        <div
                animate={{ y: [0, -10, 0], opacity: [0.3, 0.5, 0.3] }}
                transition={{ duration: 5, repeat: Infinity, ease: 'easeInOut' }}
                class="absolute bottom-0 left-0 right-0 h-24 bg-gradient-to-t from-blue-500/10 to-transparent"
        />
    </div>
</div>