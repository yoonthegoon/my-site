use crate::blog::models::Post;
use crate::blog::templates::{PostDetailTemplate, PostListTemplate};
use crate::templates::HtmlTemplate;
use axum::extract::Path;
use axum::response::IntoResponse;

pub async fn post_list_handler() -> impl IntoResponse {
    let template = PostListTemplate {};
    HtmlTemplate(template)
}

pub async fn post_detail_handler(Path(slug): Path<String>) -> impl IntoResponse {
    let title = String::from("Inquit incepta");
    let content = markdown::to_html("## Cura ter iam celebrabat relictis

Lorem markdownum terrae, geras suppositosque sicut. Durum mundi **Latona
felix**. Curva morte leti tulit vellem in est placido vultus es harum. Pelides
[nares](http://ubi.net/), leto mecum quae!

> Unum levitas *positos recentibus quod* perque facie Olympi similemque nata
> commune! Habuit hos non montibus herba, tenuit et interdum pectora gratia!

## Polyxena unda proles nihil nutrici dum totiens

Modo toro cornu delapsus inminet. Ense ora est cauda corpora tum sua visa,
coniunx. Thalamo speluncae carior generi movetur pulcherrima undas custos, quia
ensis ferrum sagittis tyranni Tritoniacam ictus in inexspectatus Quid. Pavit
erat est silva nondum et presso non tecta cum longa suisque ossibus: femina
submittere resedit crines madefactaque. Datum tulitquemuneris Philippi ad Iovis
flammas, soleant multas relinquite erit laboribus Atlante sunt intus serta
paulum potest.

Aequor eloquio in [pater non formosior](http://caduca.org/exsistunt-capit)
iraque, nequeo ista; ignota miserandus Ligdum subit pietas. Quo et patriumque
neve traxere licet, sublatus *et* frugum hunc erepta palmis reluxit, non sunt
non. Grani probetne aequora. Solo campus, iam trepidoque sedesque taedas
deprendere sumpserat Amyntor sustinui quem pudore editus, umbras taurus.
Phlegethontide nunc **mollita** velut numine falsa, iacentem mutum.

## Vidit e nitidaque vosque iugalis tendentem

Hic quoque se instabilemque tota ab nisi haec unda, et Cadme dignus genetrix,
fas genitor brevi purpureo. Frustraque hostibus duo hector quoque gaudent,
fiducia forsitan male modo **plura querno veniebat**. Sorores utilitas dumque.
Hae otia nec et firmatque pudori etiam ignibus diu furenti volat nec.

```javascript
tebibyteDiskCps = ttl;
if (externalNoc >= 4) {
    compression.hard(imapOsi, title_key_widget, imageVle);
    httpHashtagArt *= itunes_snapshot_joystick.moodle(bccRawPoint,
            parity_format);
    realityProgram *= sprite;
}
user.burn *= tape_secondary(degaussReal + downloadZif.mouse(
        petaflops_impression, 94, remote));
matrixCyberbullyingSnmp = 1;
```

## Decorem sed tollens flamma arma duro visendae

Coeptis et aequor: mihi tibi nat veterum fiducia formidine ut parcite rapitur.
Erat poterat: moverat non iura ensis, interdumque flebile canibusve nunc fieri.

Potest poma inter summo nostroque perosa suberant nec Asbolos dixit nec primum
correptus circuiere petis Venerem: Nile milite. Tenuique illis, positos quo
oportuit videt Iuppiter subibat, et maius genetrix litore iuvenali. Ossaque
quaeritis veterumque consuetas costas usus [color omnis](http://regnat.net/) ut,
essem, dona in recondita heros.");

    let template = PostDetailTemplate {
        post: Post {
            slug,
            title,
            content,
        },
    };
    HtmlTemplate(template)
}
