

trait AuthController {

    fn login(&self, ) -> Result<String, String>;

    fn logout(&self, token: &str) -> Result<(), String>;
}
