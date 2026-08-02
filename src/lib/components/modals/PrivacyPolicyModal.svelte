<script lang="ts">
    import BaseModal from '$lib/components/modals/BaseModal.svelte'

    let { isPrivicyPolicyOpen = $bindable() } = $props()

    const onCancel = (): void => {
        isPrivicyPolicyOpen = false
    }
</script>

<BaseModal onClose={onCancel}>
    <div class="w-full transform rounded-xl p-6" tabindex="-1">
        <h2 id="modal-title" class="mb-6 text-xl font-semibold text-textColor">Privacy Policy</h2>
        <div class="min-w-[50vw] space-y-5">
            <div>
                <ol class="list-disc pl-8">
                    <li>Only data needed for the app to function is collected.</li>
                    <li>
                        You can delete your account and all associated data at any time. This is immediate and
                        unrecoverable.
                    </li>
                    <li>Server logs containing IP addresses are automatically deleted after 14 days.</li>
                    <li>Your data is not shared or sold.</li>
                    <li>No third-party analytics or tracking.</li>
                </ol>
            </div>
            <div>
                <table class="text-textColor [&_td:nth-child(2)]:pl-36 [&_th:nth-child(2)]:pl-12">
                    <thead>
                        <tr>
                            <th>Data</th>
                            <th>Retention</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>Email address</td>
                            <td>Until account deletion</td>
                        </tr>
                        <tr>
                            <td>Username</td>
                            <td>Until account deletion</td>
                        </tr>
                        <tr>
                            <td>Password hash</td>
                            <td>Until account deletion</td>
                        </tr>
                        <tr>
                            <td>Watched media & episodes</td>
                            <td>Until account deletion or user reset</td>
                        </tr>
                        <tr>
                            <td>Collections/favorites</td>
                            <td>Until account deletion or user reset</td>
                        </tr>
                        <tr>
                            <td>Watchlist items</td>
                            <td>Until account deletion or user reset</td>
                        </tr>
                        <tr>
                            <td>Ratings</td>
                            <td>Until account deletion or user reset</td>
                        </tr>
                        <tr>
                            <td>Progress/episode tracking</td>
                            <td>Until account deletion or user reset</td>
                        </tr>
                        <tr>
                            <td>Media requests</td>
                            <td>7 days or User Deletion</td>
                        </tr>
                        <tr>
                            <td>Reports</td>
                            <td>Emailed</td>
                        </tr>
                        <tr>
                            <td>IP address</td>
                            <td>14 days</td>
                        </tr>
                        <tr>
                            <td>User agent</td>
                            <td>14 days</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</BaseModal>
