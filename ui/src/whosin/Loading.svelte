<script>
// Prop for adjustable size with a default value
export let size = 28;
</script>

<style>
.spinner {
    animation: spin 5s linear infinite;
}
@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}
</style>

<svg class="spinner" width="{size}" height="{size}" viewBox="0 0 50 50">
<circle cx="25" cy="25" r="20" fill="none" stroke="currentColor" stroke-width="5"
        stroke-dasharray="94.2" stroke-dashoffset="0">
    <animateTransform attributeName="transform"
                    type="rotate"
                    from="0 25 25"
                    to="360 25 25"
                    dur="1s"
                    repeatCount="indefinite"/>
</circle>
</svg>