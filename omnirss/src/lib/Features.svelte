<script>
    import Fa from 'svelte-fa'
    import { faGithub } from '@fortawesome/free-brands-svg-icons';
    import { faLock } from '@fortawesome/free-solid-svg-icons';
    import { faPuzzlePiece } from '@fortawesome/free-solid-svg-icons';
</script>

<main>
    <section class="text-center bg-light features-icons">
        <div class="container">
            <div class="row">
                <div class="col-lg-4">
                    <div class="mx-auto features-icons-item mb-5 mb-lg-0 mb-lg-3">
                        <div class="icon"><Fa icon={faPuzzlePiece} rotate="-45" translateY="{-0.1}" /></div>
                        <h3>Bring Your Own Client</h3>
                        <p class="lead mb-0">Works with any podcatcher / RSS client.</p>
                    </div>
                </div>
                <div class="col-lg-4">
                    <div class="mx-auto features-icons-item mb-5 mb-lg-0 mb-lg-3">
                        <div class="icon"><Fa icon={faLock} translateX="{0.03}"/></div>
                        <h3>Private</h3>
                        <p class="lead mb-0">No invasive trackers. No ads.</p>
                    </div>
                </div>
                <div class="col-lg-4">
                    <div class="mx-auto features-icons-item mb-5 mb-lg-0 mb-lg-3">
                        <div class="icon"><Fa icon={faGithub} /></div>
                        <h3>Open Source</h3>
                        <p class="lead mb-0"><a href="https://github.com/sam-myers/omnirss/">Source Code.</a> Permissively licensed.</p>
                    </div>
                </div>
            </div>
        </div>
    </section>
</main>

<style>
    .icon {
        font-size: 4.5rem;
        color: rgb(0, 123, 255);
    }
</style>
