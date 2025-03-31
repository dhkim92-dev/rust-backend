
trait MemberCreateUseCase: Interface {
    async fn create(&self, command: MemberCreateCommand) -> Result<MemberCreateCommandResult, ErrorCode>;
}

