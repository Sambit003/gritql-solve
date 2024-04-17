use crate::{
    binding::Binding,
    context::ExecContext,
    marzano_resolved_pattern::MarzanoResolvedPattern,
    pattern::{
        dynamic_snippet::DynamicPattern,
        patterns::{CodeSnippet, Matcher, Pattern, PatternName},
        resolved_pattern::ResolvedPattern,
        state::State,
        MarzanoContext,
    },
    problem::MarzanoQueryContext,
};
use anyhow::Result;
use marzano_language::language::SortId;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct MarzanoCodeSnippet {
    pub(crate) patterns: Vec<(SortId, Pattern<MarzanoQueryContext>)>,
    pub(crate) source: String,
    pub(crate) dynamic_snippet: Option<DynamicPattern<MarzanoQueryContext>>,
}

impl MarzanoCodeSnippet {
    pub fn new(
        patterns: Vec<(SortId, Pattern<MarzanoQueryContext>)>,
        dynamic_snippet: Option<DynamicPattern<MarzanoQueryContext>>,
        source: &str,
    ) -> Self {
        Self {
            patterns,
            source: source.to_string(),
            dynamic_snippet,
        }
    }
}

impl CodeSnippet<MarzanoQueryContext> for MarzanoCodeSnippet {
    fn patterns(&self) -> impl Iterator<Item = &Pattern<MarzanoQueryContext>> {
        self.patterns.iter().map(|p| &p.1)
    }

    fn dynamic_snippet(&self) -> Option<&DynamicPattern<MarzanoQueryContext>> {
        self.dynamic_snippet.as_ref()
    }
}

impl PatternName for MarzanoCodeSnippet {
    fn name(&self) -> &'static str {
        "CODESNIPPET"
    }
}

impl Matcher<MarzanoQueryContext> for MarzanoCodeSnippet {
    // wrong, but whatever for now
    fn execute<'a>(
        &'a self,
        resolved: &MarzanoResolvedPattern<'a>,
        state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let Some(binding) = resolved.get_last_binding() else {
            return Ok(resolved.text(&state.files, context.language())?.trim() == self.source);
        };

        let Some(node) = binding.singleton() else {
            return Ok(false);
        };

        if let Some((_, pattern)) = self
            .patterns
            .iter()
            .find(|(id, _)| *id == node.node.kind_id())
        {
            pattern.execute(resolved, state, context, logs)
        } else {
            Ok(false)
        }
    }
}
