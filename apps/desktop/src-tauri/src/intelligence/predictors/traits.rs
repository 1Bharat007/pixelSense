use crate::intelligence::models::IntelligenceContext;

/// The PredictionProvider trait acts as a placeholder boundary for future
/// machine learning models. 
/// 
/// Future local AI models will implement this trait to plug into the 
/// IntelligenceManager without modifying core orchestration logic.
pub trait PredictionProvider: Send + Sync {
    /// Given the current context, what is the predicted most comfortable brightness?
    fn predict_optimal_brightness(&self, context: &IntelligenceContext) -> Option<u8>;
    
    /// Given the user's history, predict if they are about to start a deep focus session.
    fn predict_deep_focus(&self, context: &IntelligenceContext) -> f32; // 0.0 to 1.0 confidence
}
